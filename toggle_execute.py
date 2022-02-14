import argparse


def get_args():
    """Parse argument given to program"""
    parser = argparse.ArgumentParser(description="Toggle execute gute nacht")
    parser.add_argument(
        "-n", "--no-execute", action="store_true", help="Don't execute gute nacht today"
    )
    parser.add_argument(
        "-e", "--execute", action="store_true", help="Execute gute nacht today"
    )

    return parser.parse_args().__dict__


EXECUTE_TRUE = "EXECUTE_TODAY: bool = True\n"
EXECUTE_FALSE = "EXECUTE_TODAY: bool = False\n"


def toggle(yes=True, neutral=True):
    """Toggle the config file"""
    with open("./config.py", "r+") as conf:
        conf_lines = list(conf)

    conf_lines[-1] = (
        (EXECUTE_FALSE if conf_lines[-1] == EXECUTE_TRUE else EXECUTE_TRUE)
        if neutral
        else (EXECUTE_TRUE if yes else EXECUTE_FALSE)
    )

    with open("./config.py", "w") as conf:
        conf.writelines(conf_lines)


def main():
    args = get_args()
    ex = args["execute"]
    no_ex = args["no_execute"]
    neutral = not ex and not no_ex
    if ex and no_ex:
        print("Invalid options!")
        return
    yes = ex
    toggle(yes=yes, neutral=neutral)


if __name__ == "__main__":
    main()
