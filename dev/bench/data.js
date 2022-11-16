window.BENCHMARK_DATA = {
  "lastUpdate": 1668574260408,
  "repoUrl": "https://github.com/modern-js-dev/swc-plugins",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "1114550440@qq.com",
            "name": "fengyu",
            "username": "JSerFeng"
          },
          "committer": {
            "email": "1114550440@qq.com",
            "name": "Fy",
            "username": "JSerFeng"
          },
          "distinct": true,
          "id": "ee3aa41e1bcaa5608a051dda502aa05eb9f7dc01",
          "message": "bench: compare with @swc/core minify",
          "timestamp": "2022-11-16T12:30:52+08:00",
          "tree_id": "fc99736b4fae1fd95f4672790c17934f03bcb828",
          "url": "https://github.com/modern-js-dev/swc-plugins/commit/ee3aa41e1bcaa5608a051dda502aa05eb9f7dc01"
        },
        "date": 1668574259693,
        "tool": "cargo",
        "benches": [
          {
            "name": "minify_large_bundle_no_sourcemap",
            "value": 929664917,
            "range": "± 37510246",
            "unit": "ns/iter"
          },
          {
            "name": "minify_large_bundle_with_sourcemap",
            "value": 1026542668,
            "range": "± 32559937",
            "unit": "ns/iter"
          },
          {
            "name": "swc_core_minify",
            "value": 1034241743,
            "range": "± 27614252",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}