
def print_vars():
    vars_from_config = {
        'config': {
          'a': "{{config.a}}",
          'b': "{{config.b}}",
          'c': {{config.c}},
          'd': {{config.d}},
          'e': {{config.e}},
          'f': {{config.f}},
          'nested': {
            'path': {
                'fullpath': [
                    "{{config.nested.path.fullpath[0]}}",
                    "{{config.nested.path.fullpath[1]}}",
                    "{{config.nested.path.fullpath[2]}}"
                ]
            },
          },
          'config_interpolation': "{{config_interpolation}}"
        },
    }

    for k,v in vars_from_config.items():
        print("{}: {}".format(k,v))

if __name__ == "__main__":
    print_vars()
