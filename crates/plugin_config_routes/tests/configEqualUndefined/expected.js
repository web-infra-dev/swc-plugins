var a = {
    component: "a's name"
};
var b = {
    "component": "b's name"
};
export default [
    {
        path: "/",
        component: "@/home/Layout",
        title: "home",
        routes: [
            {
                path: "/apple",
                component: "@/component/Apple",
                title: "apple",
                exact: true
            }
        ]
    },
    {
        path: "*",
        component: "@/component/404",
        title: "404"
    }
];
