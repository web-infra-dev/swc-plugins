var a = {
    component: require("a's name").default,
    "module": require("a's name")
};
var b = {
    "component": require("b's name").default,
    "module": require("b's name")
};
export default [
    {
        path: "/",
        component: require("@/home/Layout").default,
        title: "home",
        routes: [
            {
                path: "/apple",
                component: require("@/component/Apple").default,
                title: "apple",
                exact: true,
                "module": require("@/component/Apple")
            }
        ],
        "module": require("@/home/Layout")
    },
    {
        path: "*",
        component: require("@/component/404").default,
        title: "404",
        "module": require("@/component/404")
    }
];
