# Forgery

A Makefile clone but with a few more features. It allows you to
run commands you would normally do in a terminal but specify shortcut
commands to use like a Makefile. It also allows you to run commands
before each and every command you specified using the .init key.

<!-- TODO: handle case when no .init key was passed in -->
<!-- for now, .init key is required -->

<!--
TODO: add colored output
TODO: when arg len is 1, display all forgefile.json keys as possible commands
TODO: --help command
TODO: crates.io documentation
TODO: nested commands support
TODO: allow no need for a forgefile.json file
-->

## How to use

1. Create a `forgefile.json` file in the root of your project.

    ```json
    {
        ".init": ["source venv/bin/activate"],
        "install": ["pip install -r requirements.txt"],
        "print": ["echo 'Hello World!'"]
    }
    ```

    The `.init` key is required. The rest of the keys are optional.
    In a future release, the `.init` key will be optional. It will
    run before each command in the list of commands provided per key.

2. Install the latest version of `forge` executable [here](https://github.com/apinanyogaratnam/forgery/releases/download/v1.0.0/forgery).

3. Add forge to your PATH.

4. Run `forge` in your project directory.

    ```bash
    forge install
    forge pip freeze
    ```
