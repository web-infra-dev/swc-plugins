function render() {
  return <foo />;
}

function render({text}) {

  setTimeout(() => {
    text = 1;
  })

  return function () {
    return console.log(text);
  };
}

const ret = render();
ret()

setTimeout(() => {
  ret()
}, 200)

