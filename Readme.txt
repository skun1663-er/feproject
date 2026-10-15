cargo --version
rustc --version
https://doc.rust-lang.org/cargo/reference/manifest.html

cargo run

main.rs contains your program.
Cargo.toml contains your project's configuration and dependencies.
Cargo builds and runs the program.

# Short form
rustc -V

# Cargo version (tracks rustc)
cargo --version

# List all installed toolchains
rustup toolchain list

# Show the currently active toolchain
rustup show active-toolchain

# Find the rustc binary path (confirms which toolchain is in use)
rustup which rustc

# The idiomatic way to handle multiple runnable entry points in Cargo is to put them in the src/bin/ directory. Every file inside this directory becomes its own standalone executable, provided it contains a fn main(). Use command: cargo run --bin file_name

# If you are writing standalone demonstrative scripts or examples that showcase your project's features, place them in an examples/ directory at the root level of your project. Use command: cargo run --example file_name

# If you prefer to keep your runnable file anywhere else in your project structure, you can explicitly register it as a binary target within your Cargo.toml configuration.
example: .toml file:
[[bin]]
name = "custom_run"
path = "src/anywhere/my_file.rs"

Use command: cargo run --bin custom_run
=============================================================================
1) Hello world:-
Printing output:
fn main() {
     println!("Hello, world!");
	}
 and running a Rust program: cargo run --bin [file_name]

2) Rust Calculator:-
Once the calculator works, try these improvements:
1. Allow the user to calculate repeatedly without restarting the program.
2. Add a remainder operator (%).
3. Add an option to exit the calculator.
4. Display results rounded to two decimal places.

3) Random Number Guessing Game:-
Rust's standard library doesn't provide a general-purpose random-number generator.
Added the 'rand' library.

let - Creates a variable · Throughout main.rs · Stores values
mut - Allows mutation · let mut input · Reading input modifies the string
loop - Repeats execution · Around the guessing logic · Keeps the game running
match - Handles alternatives · When parsing input · Prevents invalid input from crashing the game
continue - Starts the next iteration · On invalid input · Skips the remaining loop body
break - Exits a loop · After a correct guess · Ends the game

4) Expense Tracker:-
What: parse::<f64>() explicitly converts the input into a 64-bit floating-point number.
Where: Inside the add_expense() function.
Why: Rust needs the concrete type to resolve methods such as is_finite() on the parsed value.

Lesson: Type annotations on variables don't always provide enough context for every generic method; parse::<f64>() explicitly supplies the type parameter.

5) File Organizer:-
env::args() - Reads command-line arguments · main() · Lets you choose a folder at runtime
PathBuf - Stores a filesystem path · main() · Handles paths safely across operating systems
read_dir() - Reads folder entries · organize_folder() · Finds files to organize
extension() - Extracts a file extension · Inside the loop · Determines the file category
create_dir_all() - Creates destination folders · Before moving files · Ensures category folders exist
rename() - Moves a file · At the end of each iteration · Organizes it into its destination
io::Result - Represents success or an I/O error · Function return type · Allows filesystem failures to be reported

Creates three dummy files.
Where: C:\RustTest\Inbox.
Why: You can test file movements without risking real data; these are placeholder files, not valid media.
New-Item -ItemType Directory -Force C:\RustTest\Inbox
Set-Content C:\RustTest\Inbox\notes.txt "Test document"
Set-Content C:\RustTest\Inbox\photo.png "Fake image for testing"
Set-Content C:\RustTest\Inbox\song.mp3 "Fake audio for testing"

Run organizer: cargo run --bin file_organizer -- C:\RustTest\Inbox

verify: Get-ChildItem C:\RustTest\Inbox -Recurse

6) Dry run file_organizer:-
--dry-run previews which files would be moved without changing anything.
It makes file automation safer by letting you inspect the planned changes first.
Dry-run mode prints planned moves, while normal mode moves files.
You can check the results before making changes to your files.

Preview only — files should remain in their original locations:
cargo run --bin file_organizer -- C:\RustTest\Inbox --dry-run
Actually move the files:
cargo run --bin file_organizer -- C:\RustTest\Inbox
Remember: use --dry-run to preview and omit it to move files.

7)Automated Unit Testing:-
Adds five test functions covering file categories and uppercase extensions.
assert_eq! checks the actual category against the expected category and fails the test if they differ.
Run the organizer again: cargo run --bin file_organizer_preview -- C:\RustTest\Inbox --dry-run

8) Log File Analyzer:-
Goal: Build a tool that reads a log file, counts errors and warnings, and summarizes the results.
Why: Log analysis is a practical foundation for debugging, monitoring, and automation testing.
Writes seven log entries into app.log: 
Set-Content C:\RustTest\app.log -Value @("INFO: Application started", "WARNING: Slow response", "ERROR: Database connection failed", "INFO: User logged in", "ERROR: Request failed", "WARNING: Memory usage high", "INFO: Application stopped")

9) Website Availability Checker:-
What: Build a Rust tool that checks whether a website responds to an HTTP request.
Why: Website availability checks are useful for monitoring websites and building automated testing tools.
What: Adds reqwest with blocking HTTP support:
cargo add reqwest@0.13.5 --features blocking

10) Website Response-Time Tester:-
Remember: This measures time until the HTTP response headers arrive, not the time required to download the entire page or render it in a browser.

11) Multi-Website Availability Checker:-
Goal: Check multiple websites in a single run and report their HTTP status and response time.
What: The vector stores three URLs, and the loop checks each one.
Where: The urls vector and for url in &urls loop.
Why: You can monitor several websites without repeating the same request code.
......What: Adds a browser-like User-Agent header.
Where: The reqwest::blocking::Client::builder() section.
Why: It tests whether Wikipedia's response is affected by the request's user-agent identity; it does not guarantee access.

12)Export Monitoring Results to CSV:-
What: Save each website's URL, HTTP status, response time, and result into a CSV report.
Why: CSV reports can be opened in Excel and used to track website availability over time.
Open the report: Invoke-Item .\website_report.csv
Note: This creates a new report each run, replacing the previous report. Once it works, we'll upgrade it to preserve historical results rather than overwrite them.

13) Website Monitor with Historical Records:-
Goal: Upgrade your CSV monitor so every run appends new results instead of overwriting previous records.
What: You'll learn file appending, timestamps, and persistent monitoring history.
Why: Historical data helps you compare response times and identify recurring website failures.
Note: The timestamp is stored as Unix time (seconds since January 1, 1970, UTC), and the file is created relative to the directory from which you run Cargo. This example assumes URLs do not contain commas or quotation marks.

14)
