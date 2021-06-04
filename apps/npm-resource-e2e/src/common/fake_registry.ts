import { MockServerClient } from 'mockserver-client/mockServerClient';
import { mockServerClient } from 'mockserver-client';
import { readFileSync } from 'fs';
import { RequestDefinition } from 'mockserver-client/mockServer';

export interface PackagePreparationConfig {
  name: string;
  version: string;
  artifactPath: string;
}

export interface PrepareLoginConfig {
  password: string;
  username: string;
  token: string;
}

interface PrepareUploadConfig {
  package: string;
  token?: string;
}

export class FakeRegistry {
  constructor(
    public host: string,
    public port: number,
    public mockserver: MockServerClient
  ) {}

  static async connect({ host, port }): Promise<FakeRegistry> {
    const client = await mockServerClient(host, port);
    return new FakeRegistry(host, port, client);
  }

  async preparePackage(packagePreparationConfig: PackagePreparationConfig) {
    const tarPath = `/${packagePreparationConfig.name}/-/${packagePreparationConfig.name}-${packagePreparationConfig.version}.tgz`;
    const tarUrl = `http://${this.host}:${this.port}${tarPath}`;

    const versions = {};
    versions[packagePreparationConfig.version] = { dist: { tarball: tarUrl } };

    await this.mockserver.mockAnyResponse({
      httpRequest: {
        method: 'GET',
        path: `/${packagePreparationConfig.name}`,
      },
      httpResponse: {
        statusCode: 200,
        body: {
          name: packagePreparationConfig.name,
          'dist-tags': {},
          versions,
          time: {
            modified: '2021-03-05T16:07:58.024Z',
            created: '2021-03-04T21:55:06.634Z',
            '0.0.0': '2021-03-04T21:55:06.634Z',
          },
        },
      },
    });

    await this.mockserver.mockAnyResponse({
      httpRequest: {
        method: 'GET',
        path: tarPath,
      },
      httpResponse: {
        statusCode: 200,
        headers: [
          {
            name: 'content-type',
            values: ['application/octet-stream'],
          },
        ],
        body: {
          type: 'BINARY',
          base64Bytes: readFileSync(packagePreparationConfig.artifactPath, {
            encoding: 'base64',
            flag: 'r',
          }),
        },
      },
    });
  }

  async prepareLogin(param: PrepareLoginConfig) {
    await this.mockserver.mockAnyResponse({
      httpRequest: {
        method: 'PUT',
        path: `/-/user/org.couchdb.user:${param.username}/-rev/undefined`,
        body: {
          name: param.username,
          password: param.password,
        },
      },
      httpResponse: {
        statusCode: 200,
        body: {
          token: param.token,
        },
      },
    });
  }

  async prepareUpload(params: PrepareUploadConfig) {
    const httpRequest: RequestDefinition = {
      method: 'PUT',
      path: `/${params.package}`,
    };

    if (params.token) {
      httpRequest.headers = [
        {
          name: 'Authorization',
          values: [`Bearer ${params.token}`],
        },
      ];
    }

    await this.mockserver.mockAnyResponse({
      httpRequest,
      httpResponse: {
        statusCode: 200,
        body: {},
      },
    });
  }

  async verifyPackageDownloadCalled(
    packageName: string,
    version: string,
    token?: string
  ) {
    const def: RequestDefinition = {
      method: 'GET',
      path: `/${packageName}/-/${packageName}-${version}.tgz`,
    };
    if (token) {
      def.headers = [
        {
          name: 'Authorization',
          values: [`Bearer ${token}`],
        },
      ];
    }
    return this.mockserver.verify(def);
  }

  async verifyPackageUploaded(packageName: string, token?: string) {
    const def: RequestDefinition = {
      method: 'PUT',
      path: `/${packageName}`,
    };
    if (token) {
      def.headers = [
        {
          name: 'Authorization',
          values: [`Bearer ${token}`],
        },
      ];
    }
    return this.mockserver.verify(def);
  }

  async verifyPackageManifestCalled(packageName: string, token?: string) {
    const def: RequestDefinition = {
      method: 'GET',
      path: `/${packageName}`,
    };
    if (token) {
      def.headers = [
        {
          name: 'Authorization',
          values: [`Bearer ${token}`],
        },
      ];
    }
    return this.mockserver.verify(def);
  }

  async verifyLoginCalled(
    username: string,
    password: string,
    basicToken: string
  ) {
    return this.mockserver.verify({
      method: 'PUT',
      path: `/-/user/org.couchdb.user:${username}/-rev/undefined`,
      headers: [
        {
          name: 'Authorization',
          values: [`BASIC ${basicToken}`],
        },
      ],
      body: {
        name: username,
        password: password,
      },
    });
  }

  async verifyLoginNotCalled() {
    return this.mockserver.verify(
      {
        method: 'PUT',
        path: `/-/user/org.couchdb.user:{username}/-rev/undefined`,
        pathParameters: {
          username: ['\\w+'],
        },
      },
      0
    );
  }
}
