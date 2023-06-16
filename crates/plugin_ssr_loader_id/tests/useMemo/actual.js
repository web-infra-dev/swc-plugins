import { useLoader } from '@modern-js/runtime';
import { memo } from 'react';

const Wrap = memo((props) => {
  useLoader(() => {
    console.log('wrap');
    return Promise.resolve({});
  }, {});

  return <div>wrap header{props.children}</div>;
});
export default Wrap;
