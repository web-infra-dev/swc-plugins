var a = {
    component: loadable(function() {
        return import("a's name");
    })
};
var b = {
    "component": loadable(function() {
        return import("b's name");
    })
};
export default [
    {
        path: "/",
        component: loadable(function() {
            return import("@/home/Layout");
        }),
        title: "home",
        routes: [
            {
                path: "/apple",
                component: loadable(function() {
                    return import("@/component/Apple");
                }),
                title: "apple",
                exact: true
            }
        ]
    },
    {
        path: "*",
        component: loadable(function() {
            return import("@/component/404");
        }),
        title: "404"
    }
];
