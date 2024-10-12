# Music Manager

Music Manager is a GTK-based desktop application that helps you manage your music library. It allows users to browse, search, and filter songs by title, artist, and album. Additionally, users can visually select a directory and load songs into the application.

## Features
 
- **Dark Mode Support**: Toggle between light and dark mode.
- **Directory Selection**: Users can manually enter or visually select a directory to load their music files.
- **Search Functionality**: Search by song title, artist, or album using the following syntax:
     - `n:Artist Name` to search by artist.
     - `t:Song Title` to search by song title.
     - `a:Album Name` to search by album.
     - Combine multiple filters using `&&` (e.g., `n:Artist Name&&t:Song Title`).
 - **Detailed Song Information**: View detailed information such as song title, path, track number, year, and genre.
 - **Progress Bar**: Displays progress when loading songs from a directory.
 
 ## Installation
 
 To install and run the application, make sure you have the following dependencies installed:
 
 - Rust (latest version)
 - GTK 3.x development libraries
 - SQLite (for database management)
 
 You can install Rust by visiting the [official Rust website](https://www.rust-lang.org/).
 
 ### Ubuntu (or similar)
 
 Run the following commands to install the necessary dependencies:
 
 ```bash
 sudo apt update
 sudo apt install libgtk-3-dev sqlite3 libsqlite3-dev
 ```
 
 ### Fedora
 
 
 ```bash
 sudo dnf install gtk3-devel sqlite sqlite-devel
 ```

 ### Arch
 
 
 ```bash
 sudo pacman -S rust gtk3 sqlite
 ```
 
 
 ## Building the Project
 
 Clone the repository and build the project using Cargo:
 
 ```bash
 git clone https://github.com/Luisreyescc/MusicManager.git
 ```
 
 ## Running the Application
 
 To run the application after building, simply execute the following command:
 
 ```bash
 cd MusicManager
 cargo run
 ```
 
 ## Usage

 1. **Select a Directory**: You can either type the directory path or click on the folder icon to visually select a directory containing music files.
 2. **Refresh**: After selecting a directory, click the "Refresh" button to load the songs.
 3. **Search**: Use the search bar to filter songs by title (`t:`), artist (`n:`), or album (`a:`). Combine filters using `&&`.
 4. **View Song Details**: Select a song to view its detailed information in the "Song Details" pane.
 
 ## Contributing
 
 Contributions are welcome! To contribute:
 
 1. Fork the repository.
 2. Create a new branch (`git checkout -b feature-branch`).
 3. Commit your changes (`git commit -am 'Add new feature'`).
 4. Push to the branch (`git push origin feature-branch`).
 5. Create a pull request.
 .
