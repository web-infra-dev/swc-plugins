const A = Foo;
const B = Foo;
const C = Foo;
function outer(arg) {
  let _A;
  const valueB = null;
  const valueA = {};
  function inner() {
    console.log(_A || (_A = <A keyA={valueA}>
        <B keyB={valueB}>
          <C keyC={arg}/>
        </B>
      </A>));
  }
  inner();
}
