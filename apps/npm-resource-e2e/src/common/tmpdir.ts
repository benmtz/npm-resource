import { v4 as uuidv4 } from 'uuid';
import { ensureDirSync, removeSync } from 'fs-extra';
import { tmpdir } from 'os';
import { join } from 'path';

export class Tmpdir {
  constructor(public path: string) {
    ensureDirSync(this.path);
  }

  static withRandomName() {
    return new Tmpdir(join(tmpdir(), uuidv4()));
  }

  dispose() {
    removeSync(this.path);
  }
}
