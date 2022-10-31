const FooBasic = ()=>/*#__PURE__*/ React.createElement("div", null);

const FooExtraReference = ()=>/*#__PURE__*/ React.createElement("div", null);

const FooExtraReferenceSpread = ()=>/*#__PURE__*/ React.createElement("div", null);

const FooWrapped = ()=>/*#__PURE__*/ React.createElement("div", null);

const propTypesReferenced = {
  foo: PropTypes.string
};

const FooReferenced = ()=>/*#__PURE__*/ React.createElement("div", {
  bar: propTypesReferenced
});

export const propTypesExported = {
  foo: PropTypes.string
};

const FooExported = ()=>/*#__PURE__*/ React.createElement("div", null);