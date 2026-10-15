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

14) Website Uptime Summary:-
What: Build a tool that calculates each website’s success rate from your historical CSV records.
Why: Learn file reading, CSV processing, HashMaps, and reliability metrics used in monitoring tools.
Keeping it separate prevents changes to your working projects.
Run it. What: Compile and execute the uptime calculator. Where: PowerShell, from the folder containing Cargo.toml. Why: It reads the history file generated by Project 13.
Expected result: Each website should display its successful-check count and uptime percentage.
Note: This beginner version expects the CSV format generated by Project 13 and counts HTTP 2xx responses as successful. Run Project 13 at least once first so website_history.csv exists.

15) Website Change Detector:-
What: Build a Rust tool that detects when a website's page content changes between checks.
Why: Learn HTTP requests, string comparison, file persistence, and a technique used in website monitoring.
The first run saves a baseline; later runs compare against it.
Important: This compares the entire HTML response, so even small changes such as timestamps or rotating content can trigger an alert.

16) Website Keyword Monitor:-
What: Build a Rust tool that checks whether specific keywords appear on a website.
Why: Learn text searching, case-insensitive matching, HTTP requests, and simple website content monitoring.
It downloads the page and checks each keyword against its HTML content.
Understand the result:
- [FOUND] example — the word appears in the downloaded page.
- [MISSING] rust — the word does not appear in the downloaded page.
- to_lowercase() — makes matching case-insensitive.
Limitation: This checks the downloaded HTML, not text rendered later by JavaScript.

17) Broken Link Checker:-
What: Build a Rust tool that extracts links from a webpage and checks whether they respond successfully.
Why: Learn HTML link extraction, URL handling, HTTP status checking, and automated website testing.
Add the scraper library. It lets Rust extract links from HTML without manually parsing tags: cargo add scraper
What: Extract links from a page and test each link.
Why: The program resolves relative links, requests each URL, and reports HTTP errors.
The tool scans the example webpage and tests its discovered links.
Understand the output:
- [OK] — the link returned a successful HTTP status.
- [HTTP ERROR] — the server responded with a non-success status, such as 404.
- [REQUEST FAILED] — the request failed due to a timeout, connection problem, or another request error.
Important: A link returning an error isn't always broken; some websites block automated requests or require authentication. This beginner version checks links sequentially and doesn't crawl beyond the initial page.

18) Website Security Header Checker:-
What: Build a Rust tool that checks a website's HTTP response headers for common security protections.
Why: Learn HTTP headers and basic web security auditing.
Understand the output:
- [PRESENT] — the header exists in the server's response.
- [MISSING] — the header was not found.
- Headers present: 4/6 — four of the six checked headers exist.
Important: A missing header is not automatically a vulnerability; the right protections depend on the website. This tool checks header presence, not whether every header value is configured securely.

19) TCP Port Checker:-
What: Build a Rust tool that checks whether specific TCP ports accept connections on your own computer.
Why: Learn TCP networking, socket connections, timeouts, and basic troubleshooting used in network automation.
Understand the output:
- [OPEN] — a service accepted the TCP connection.
- [CLOSED or FILTERED] — the connection failed or timed out; this doesn't distinguish a closed port from a firewall filter.
- 127.0.0.1 — your own computer, so you can test safely without scanning other machines.
Practice: Start a local development server on port 3000, run the checker again, and observe the result.

20) System Resource Monitor:-
What: Build a Rust tool that displays your computer's CPU and memory usage.
Why: Learn system monitoring, external libraries, and resource measurements used in performance testing and automation.
Since you're using Windows, we'll use the cross-platform Rust library sysinfo. Add sysinfo to your project dependencies. This library reads CPU and memory information from Windows and other supported operating systems.
What: The code retrieves CPU and RAM statistics.
Where: The main() function in system_monitor.rs.
Why: refresh_all() updates the measurements, and dividing bytes by \(1{,}073{,}741{,}824\) converts them to GiB. CPU readings rely on refreshed measurements.

21) Live System Monitor:-
What: Build a Rust program that continuously displays CPU and RAM usage.
Why: Learn repeated sampling, loops, screen refreshing, and real-time performance monitoring.
The loop refreshes the measurements every second and redraws the display.
Understand the important parts:
- loop — repeats the monitoring process indefinitely.
- refresh_cpu_usage() — requests an updated CPU measurement.
- refresh_memory() — refreshes RAM statistics.
- thread::sleep() — waits one second between readings.
- Ctrl+C — stops the program.
Expected result: CPU and memory readings update approximately once per second.

22) System Alert Monitor:-
Build a Rust tool that warns you when CPU or RAM usage exceeds a specified threshold.
Why: Learn threshold-based alerts, conditional logic, and monitoring patterns used in automated testing and production systems.
It checks resource usage every second and prints an alert when either threshold is reached.
Test the alerts:
What: Temporarily change let cpu_limit = 80.0; to let cpu_limit = 1.0;.
Where: At the top of system_alert_monitor.rs.
Why: A low threshold makes it easier to verify that the alert logic works; restore it to 80.0 afterward.

23) Running Process Viewer:-
Build a Rust tool that lists running processes, their CPU usage, and memory consumption.
Learn process enumeration and resource inspection—useful for system diagnostics and automation testing.
It collects process information and sorts processes by memory usage.
Understand the output:
- PROCESS: The running process's name.
- CPU (%): The process's reported CPU usage; values can vary and may exceed 100% for multi-core usage.
- MEMORY (MB): Memory currently attributed to the process, converted from bytes.
- Top 20: Only the 20 processes using the most memory are displayed.
Note: Some Windows processes may have restricted or incomplete information, depending on permissions.

24) Live Process Monitor:-
Build a Rust tool that continuously monitors running processes and their CPU and memory usage.
Learn repeated process sampling, sorting, and real-time system diagnostics.
It refreshes process information and displays the 15 processes currently using the most memory.
Understand the key parts:
- refresh_all() — refreshes system and process information.
- sort_by() — orders processes by memory consumption.
- take(15) — limits the display to 15 processes.
- thread::sleep() — waits two seconds before refreshing again.
- Ctrl+C — stops the program.
Note: The first CPU readings may be zero or inaccurate because CPU usage needs time between measurements to become meaningful.
Running Process Viewer:
Launch → Collect data → Display results → Stop
Like taking a photograph of your running processes.
Live Process Monitor:
Launch → Collect → Display → Wait → Repeat
Like watching a live video of your running processes.

25) Process Search Tool:-
Build a Rust tool that searches running processes by name and displays their process ID, CPU usage, and memory consumption.
Learn string matching, process identification, and targeted system diagnostics.
The program searches the current process list for names matching your input.
Test it:
Try entering--
- chrome — searches for Chrome processes.
- code — searches for VS Code processes.
- rust — searches for processes whose names contain rust.
Understand the important parts:
- read_line() — receives your search term.
- to_lowercase() — makes matching case-insensitive.
- contains() — matches partial process names.
- pid — identifies a particular running process.
Note: CPU usage may initially show zero because this version takes a single snapshot. Some processes may also be hidden or have incomplete information due to Windows permissions.

26) Process Resource Alert Tool:-
Build a Rust tool that monitors running processes and warns when any process exceeds your CPU or memory thresholds.
Learn process-level monitoring, threshold checks, and automated resource alerts.
It repeatedly checks running processes and reports those exceeding either threshold.
Test the alerts:
What: Temporarily lower memory_limit_mb from 500.0 to 50.0.
Where: Near the beginning of process_resource_alert.rs.
Why: A lower threshold makes it easier to see alerts on ordinary processes; restore the threshold afterward.
- [ALERT] — a process exceeded at least one threshold.
- PID — identifies the process.
- CPU — reported CPU usage percentage.
- Memory — memory attributed to that process in MB.
Note: This version may print repeated alerts for the same process on every check, and CPU usage can exceed 100% on multicore systems.

27) Process Alert Logger:-
Build a Rust tool that monitors running processes and saves CPU and memory alerts to a CSV file.
It checks processes every two seconds and appends matching alerts to a CSV file.
Learn file appending, CSV logging, timestamps, and persistent monitoring records.
Verify the log:
What: Open the generated CSV file.
Where: In another PowerShell window, from the same project directory.
Why: This confirms that alerts persist after each monitoring cycle.
The file contains a Unix timestamp, process ID, process name, CPU usage, memory usage, and alert reason.
Testing tip: Temporarily lower memory_limit_mb to 50.0 to make alerts easier to trigger.
Note: This logs every matching process on every cycle, so the CSV can grow quickly. Press Ctrl+C to stop the monitor.

28)