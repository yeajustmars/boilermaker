(function() {
  let vars_from_node_profile = {
    config: {
      a: "{{config.a}}",
      b: "{{config.b}}",
      c: {{config.c}},
      d: {{config.d}},
      e: {{config.e}},
      f: {{config.f}},
      nested: {
        path: {
            fullpath: [
                "{{ config.nested.path.fullpath[0] }}",
                "{{ config.nested.path.fullpath[1] }}",
                "{{ config.nested.path.fullpath[2] }}"
            ]
        },
      },
      config_interpolation: "boilermaker:{project.name}:{project.version}",
    },
  };

  console.log("vars_from_node_profile: ", JSON.stringify(vars_from_node_profile, null, 3));
})();

