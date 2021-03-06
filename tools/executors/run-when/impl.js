'use strict';
var __awaiter =
  (this && this.__awaiter) ||
  function (thisArg, _arguments, P, generator) {
    function adopt(value) {
      return value instanceof P
        ? value
        : new P(function (resolve) {
            resolve(value);
          });
    }
    return new (P || (P = Promise))(function (resolve, reject) {
      function fulfilled(value) {
        try {
          step(generator.next(value));
        } catch (e) {
          reject(e);
        }
      }
      function rejected(value) {
        try {
          step(generator['throw'](value));
        } catch (e) {
          reject(e);
        }
      }
      function step(result) {
        result.done
          ? resolve(result.value)
          : adopt(result.value).then(fulfilled, rejected);
      }
      step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
  };
var __generator =
  (this && this.__generator) ||
  function (thisArg, body) {
    var _ = {
        label: 0,
        sent: function () {
          if (t[0] & 1) throw t[1];
          return t[1];
        },
        trys: [],
        ops: [],
      },
      f,
      y,
      t,
      g;
    return (
      (g = { next: verb(0), throw: verb(1), return: verb(2) }),
      typeof Symbol === 'function' &&
        (g[Symbol.iterator] = function () {
          return this;
        }),
      g
    );
    function verb(n) {
      return function (v) {
        return step([n, v]);
      };
    }
    function step(op) {
      if (f) throw new TypeError('Generator is already executing.');
      while (_)
        try {
          if (
            ((f = 1),
            y &&
              (t =
                op[0] & 2
                  ? y['return']
                  : op[0]
                  ? y['throw'] || ((t = y['return']) && t.call(y), 0)
                  : y.next) &&
              !(t = t.call(y, op[1])).done)
          )
            return t;
          if (((y = 0), t)) op = [op[0] & 2, t.value];
          switch (op[0]) {
            case 0:
            case 1:
              t = op;
              break;
            case 4:
              _.label++;
              return { value: op[1], done: false };
            case 5:
              _.label++;
              y = op[1];
              op = [0];
              continue;
            case 7:
              op = _.ops.pop();
              _.trys.pop();
              continue;
            default:
              if (
                !((t = _.trys), (t = t.length > 0 && t[t.length - 1])) &&
                (op[0] === 6 || op[0] === 2)
              ) {
                _ = 0;
                continue;
              }
              if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) {
                _.label = op[1];
                break;
              }
              if (op[0] === 6 && _.label < t[1]) {
                _.label = t[1];
                t = op;
                break;
              }
              if (t && _.label < t[2]) {
                _.label = t[2];
                _.ops.push(op);
                break;
              }
              if (t[2]) _.ops.pop();
              _.trys.pop();
              continue;
          }
          op = body.call(thisArg, _);
        } catch (e) {
          op = [6, e];
          y = 0;
        } finally {
          f = t = 0;
        }
      if (op[0] & 5) throw op[1];
      return { value: op[0] ? op[1] : void 0, done: true };
    }
  };
var __asyncValues =
  (this && this.__asyncValues) ||
  function (o) {
    if (!Symbol.asyncIterator)
      throw new TypeError('Symbol.asyncIterator is not defined.');
    var m = o[Symbol.asyncIterator],
      i;
    return m
      ? m.call(o)
      : ((o =
          typeof __values === 'function' ? __values(o) : o[Symbol.iterator]()),
        (i = {}),
        verb('next'),
        verb('throw'),
        verb('return'),
        (i[Symbol.asyncIterator] = function () {
          return this;
        }),
        i);
    function verb(n) {
      i[n] =
        o[n] &&
        function (v) {
          return new Promise(function (resolve, reject) {
            (v = o[n](v)), settle(resolve, reject, v.done, v.value);
          });
        };
    }
    function settle(resolve, reject, d, v) {
      Promise.resolve(v).then(function (v) {
        resolve({ value: v, done: d });
      }, reject);
    }
  };
exports.__esModule = true;
var devkit_1 = require('@nrwl/devkit');
var waitOn = require('wait-on');
function startDevServer(t, context) {
  return __awaiter(this, void 0, void 0, function () {
    var _a, project, target, configuration, _output;
    return __generator(this, function (_b) {
      switch (_b.label) {
        case 0:
          devkit_1.logger.debug('parsing ' + t.target);
          (_a = devkit_1.parseTargetString(t.target)),
            (project = _a.project),
            (target = _a.target),
            (configuration = _a.configuration);
          _output = devkit_1.runExecutor(
            { project: project, target: target, configuration: configuration },
            {},
            context
          );
          devkit_1.logger.info('ran ' + t.target + ' successfully');
          return [
            4 /*yield*/,
            waitOn({
              resources: [t.waitOnResource],
            }),
          ];
        case 1:
          _b.sent();
          return [2 /*return*/];
      }
    });
  });
}
function createBuilder(options, context) {
  var e_1, _a;
  return __awaiter(this, void 0, void 0, function () {
    var _b, project, target, configuration, _c, _d, output, e_1_1;
    return __generator(this, function (_e) {
      switch (_e.label) {
        case 0:
          devkit_1.logger.info('Executing "run-when"...');
          devkit_1.logger.debug('Options: ' + JSON.stringify(options, null, 2));
          return [
            4 /*yield*/,
            Promise.all(
              options.runAndWaitForTargets.map(function (runThis) {
                return startDevServer(runThis, context);
              })
            ),
          ];
        case 1:
          _e.sent();
          devkit_1.logger.debug('everything is up we can start main');
          (_b = devkit_1.parseTargetString(options.target)),
            (project = _b.project),
            (target = _b.target),
            (configuration = _b.configuration);
          _e.label = 2;
        case 2:
          _e.trys.push([2, 8, 9, 14]);
          return [
            4 /*yield*/,
            devkit_1.runExecutor(
              {
                project: project,
                target: target,
                configuration: configuration,
              },
              {},
              context
            ),
          ];
        case 3:
          _c = __asyncValues.apply(void 0, [_e.sent()]);
          _e.label = 4;
        case 4:
          return [4 /*yield*/, _c.next()];
        case 5:
          if (!((_d = _e.sent()), !_d.done)) return [3 /*break*/, 7];
          output = _d.value;
          if (!output.success) {
            throw new Error('failed');
          }
          _e.label = 6;
        case 6:
          return [3 /*break*/, 4];
        case 7:
          return [3 /*break*/, 14];
        case 8:
          e_1_1 = _e.sent();
          e_1 = { error: e_1_1 };
          return [3 /*break*/, 14];
        case 9:
          _e.trys.push([9, , 12, 13]);
          if (!(_d && !_d.done && (_a = _c['return'])))
            return [3 /*break*/, 11];
          return [4 /*yield*/, _a.call(_c)];
        case 10:
          _e.sent();
          _e.label = 11;
        case 11:
          return [3 /*break*/, 13];
        case 12:
          if (e_1) throw e_1.error;
          return [7 /*endfinally*/];
        case 13:
          return [7 /*endfinally*/];
        case 14:
          return [2 /*return*/, { success: true }];
      }
    });
  });
}
exports['default'] = createBuilder;
