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

# copy the template files from "day00/src" into the directory
cp ../day00/src/* src/
cp ../day00/README.md README.md

# If the files/test directory does not exist or no files exist in that directory, create an empty test input file and default test answers
if [ ! -d files ] || [ ! "$(ls -A files)" ]; then
  mkdir files
  touch files/test
  touch files/input
  printf "1" > files/test_answer_1
  printf "2" > files/test_answer_2
fi

# check that the project builds, and run the tests
cargo build
cargo test
