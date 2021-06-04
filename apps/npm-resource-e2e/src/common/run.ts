import { join } from 'path';
import { execSync, ExecSyncOptionsWithBufferEncoding } from 'child_process';

export class Run {
  path: string;
  payload: unknown = {};
  bin: string;

  constructor(public type: string) {}

  command(bin: 'in' | 'out' | 'check') {
    if (this.type === 'debug') {
      this.bin = join('dist', 'apps', 'npm-resource', 'debug', bin);
    } else if (this.type === 'docker') {
      this.bin = `/opt/resource/${bin}`;
    }
    return this;
  }

  get commandWithArgs() {
    if (this.type === 'docker') {
      return this.dockerizeCommand(this.bin, this.path);
    } else if (this.type === 'debug') {
      if (this.path) {
        return `${this.bin} ${this.path}`;
      }
      return this.bin;
    }
  }

  private dockerizeCommand(binaryPath: string, path?: string) {
    if (path) {
      return `docker run --rm -i --network=host -v ${path}:${path} benmtz/npm-resource:latest ${binaryPath} ${path}`;
    }
    return `docker run --rm -i --network=host benmtz/npm-resource:latest ${binaryPath}`;
  }

  execSync() {
    const options: ExecSyncOptionsWithBufferEncoding = {
      input: JSON.stringify(this.payload),
    };
    return execSync(this.commandWithArgs, options);
  }
}
