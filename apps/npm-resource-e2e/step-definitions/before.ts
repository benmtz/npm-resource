import { Before } from '@cucumber/cucumber';
import { FakeRegistry } from '../src/common/fake_registry';

Before(async () => {
  const registry = await FakeRegistry.connect({
    host: 'localhost',
    port: 1080,
  });
  await registry.mockserver.reset();
});
