# check that there is an argument
if [ $# -eq 0 ]; then
  echo "Usage: $0 <xx>\n\twhere <xx> is a two digit day number"
  exit 1
fi

# check that the argument is a number
if [ ! $1 -eq $1 2>/dev/null ]; then
  echo "Usage: $0 <xx>\n\twhere <xx> is a two digit day number"
  exit 1
fi

# check that the argument is a two digit number
if [ ! ${#1} -eq 2 ]; then
  echo "Usage: $0 <xx>\n\twhere <xx> is a two digit day number"
  exit 1
fi

# create the directory "dayxx" if it doesn't exist
if [ ! -d "day$1" ]; then
  mkdir "day$1"
  echo "Directory day$1 created"
else
  echo "Directory day$1 already exists"
fi

# use cargo to initialize the directory
cd "day$1"
cargo init --bin

# check if "advent" is a dependency in the "Cargo.toml" file
if grep -q advent Cargo.toml; then
  echo "advent dependency already exists in Cargo.toml"
else
  sed -i '' 's/\[dependencies\]/[dependencies]\nadvent = { path = "..\/advent" }/' Cargo.toml
fi
# add "advent" to the [dependencies] section in the "Cargo.toml" file

# make sure the dependency was added correctly
if ! grep -q advent Cargo.toml; then
  echo "Error: advent dependency not added to Cargo.toml"
  exit 1
fi

# If the "src" directory is empty, copy the template files from "day00/src" into the directory
if [ ! "$(ls -A src)" ]; then
  cp ../day00/src/* src/
fi

# If the files/test directory does not exist or no files exist in that directory, create an empty test input file and default test answers
if [ ! -d files ] || [ ! "$(ls -A files)" ]; then
  mkdir files
  touch files/test
  echo "1" > files/test_answer1
  echo "2" > files/test_answer2
fi

# check that the project builds, and run the tests
cargo build
cargo test
