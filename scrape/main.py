# main.py
import importlib
import sys


class CommandManager:
    def execute_command(self, command_name):
        try:
            module = importlib.import_module(command_name)
            class_name = "".join(
                list(map(lambda x: x.capitalize(), command_name.split("_")))
            )
            command_class_name = f"{class_name}Command"
            command = getattr(module, command_class_name)()
            command.execute()
        except ModuleNotFoundError:
            print(f"Command '{command_name}' not found.")
        except AttributeError:
            print(f"Module '{command_name}' has no attribute '{command_class_name}'.")
        except Exception as e:
            print(f"An error occurred: {e}")


def main():
    if len(sys.argv) < 2:
        print("Usage: python main.py <command_name>")
        return

    command_name = sys.argv[1]
    manager = CommandManager()
    manager.execute_command(command_name)


if __name__ == "__main__":
    main()
