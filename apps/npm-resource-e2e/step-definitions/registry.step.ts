import { Given, Then, World } from '@cucumber/cucumber';
import { FakeRegistry } from '../src/common/fake_registry';
import { join } from 'path';

Given('registry is up', async function (this: World) {
  this.parameters.registry = await FakeRegistry.connect({
    host: 'localhost',
    port: 1080,
  });
});

Given(
  'registry has the following packages :',
  async function (this: World, dataTable) {
    for (const pkg of dataTable.hashes()) {
      await this.parameters.registry.preparePackage({
        name: pkg.package,
        version: pkg.version,
        artifactPath: join(__dirname, '..', 'assets', 'package.tgz'),
      });
    }
  }
);

Given(
  'registry has the following users :',
  async function (this: World, dataTable) {
    for (const user of dataTable.hashes()) {
      await this.parameters.registry.prepareLogin({
        username: user.username,
        password: user.password,
        token: user.token,
      });
    }
  }
);

Given(
  'registry is ready for the following publications :',
  async function (this: World, dataTable) {
    for (const uploadData of dataTable.hashes()) {
      await this.parameters.registry.prepareUpload({
        package: uploadData.package,
        token: uploadData.token,
      });
    }
  }
);

Then(
  /login has been called with (\S+) (\S+) (\S+)/,
  async function (
    this: World,
    username: string,
    password: string,
    token: string
  ) {
    await this.parameters.registry.verifyLoginCalled(username, password, token);
  }
);

Then(
  /(\S+) manifest has been called with (no)? ?token ?(\S+)?/,
  async function (this: World, pkg: string, noToken: boolean, token: string) {
    await this.parameters.registry.verifyPackageManifestCalled(
      pkg,
      noToken && token
    );
  }
);

Then(
  /(\S+) has been downloaded in version (\S+) with (no)? ?token ?(\S+)?/,
  async function (
    this: World,
    pkg: string,
    version: string,
    noToken: boolean,
    token: string
  ) {
    await this.parameters.registry.verifyPackageDownloadCalled(
      pkg,
      version,
      noToken && token
    );
  }
);

Then('login has not been called', async function (this: World) {
  await this.parameters.registry.verifyLoginNotCalled();
});

Then(
  /(\S+) has been uploaded with (no )?token ?(\S+)?/,
  async function (this: World, pkg: string, noToken: boolean, token: string) {
    await this.parameters.registry.verifyPackageUploaded(pkg, token);
  }
);
