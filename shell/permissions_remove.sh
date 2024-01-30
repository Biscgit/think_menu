# constants
file_path="/etc/sudoers.d/think_menu"

# remove file if exists
if [ -e "$file_path" ]; then
    rm $file_path || exit 1
fi

# finished setup
exit 0