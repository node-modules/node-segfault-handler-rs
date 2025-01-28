import assert from 'node:assert';
import path from 'node:path';
import coffee from 'coffee';


describe('test/index.test.ts', () => {
  it('should print stack to stdout', async () => {
    const res = await coffee.fork(path.join(__dirname, 'fixtures/test.js'))
      .debug()
      // FIXME coffee exit code is null
      // .expect('code', 139)
      .end();
    assert(res.stderr.match(/test\/fixtures\/test\.js:6:3/));
    assert(res.stderr.match(/listOnTimeout/));
  });
});
