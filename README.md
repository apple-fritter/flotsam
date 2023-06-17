# Flotsam - IRC Moderation Tool

## Table of Contents
1. [Introduction](#introduction)
2. [Features](#features)
3. [Installation](#installation)
4. [Usage](#usage)
5. [Prerequisites](#prerequisites)
6. [Flowchart](#flowchart)
7. [IRC Meta](#irc-meta)
   1. [IRC Repositories](#fritterz)
      1. [driftwood suite](#driftwood-suite-of-irc-analytics)
         - [driftwood utilities](#driftwood-utilities)
         - [driftwood native logging plugins](#driftwood-native-logging-plugins)
      2. [heX-Chat](#hex-chat)
      3. [IRCcloud](#IRCcloud)
      4. [weechat](#weechat)
   2. [IRC Usage Considerations](#irc-usage-considerations)
      1. [Philosophy of Use](#philosophy-of-use)
      2. [Environment](#foster-a-positive-and-inclusive-environment)
      3. [Rights and Dignity of Others](#respect-the-rights-and-dignity-of-other-users)
      4. [IRC Community and Channels](#respect-the-irc-community-and-channels)
      5. [Compatibility](#ensure-compatibility)
8. [Contributing](#contributing)
9. [Disclaimer](#disclaimer)
10. [License](#license)

---

## Introduction
Flotsam is a powerful IRC moderation tool written in Rust. It aims to supplement the "[driftwood](https://github.com/apple-fritter/driftwood)" format of IRC logs by providing efficient log parsing and moderation capabilities for IRCops, channel operators, and data scientists alike, and is meant to pair closely with [jetsam](https://github.com/apple-fritter/jetsam), to aggregate a per-user metric of flagged contributions.

The Flotsam program efficiently records and analyzes log data from IRC channels, providing valuable insights for moderation purposes. As the program processes the log files, it identifies flagged lines of content that require moderation. Additionally, it maintains a count of the flagged lines attributed to each user.

Each flagged line is meticulously recorded and attributed to the user of origin. The details are stored in a tab-separated values (TSV) log file, ensuring easy readability and data retrieval. The log file is organized alphabetically by the user's name, allowing for quick access and reference.

The recording process takes into account the entire log directory structure, recursively scanning through the log files. As the program encounters flagged lines, it increments the count associated with the respective user. This count provides valuable information about the extent of a user's contribution to content that may require suppression or further review.

By maintaining an accurate and comprehensive log of flagged lines, Flotsam enables IRC operators, channel moderators, and data scientists to gain deeper insights into user behavior and take necessary actions to ensure a positive and safe environment within IRC channels.

---

## Features
- **Driftwood Format Parsing:** Flotsam can parse IRC logs in the Driftwood format, ensuring accurate extraction of log data.
- **User Contribution Count:** Flotsam counts the number of instances each user has contributed to content that may require moderation.
- **Recursive Log Processing:** Flotsam recursively traverses log directories, enabling analysis of logs in nested subdirectories.
- **TSV Log Generation:** Flotsam generates a sorted Tab-Separated Values (TSV) log file containing user counts, facilitating further analysis.

## Installation
1. Ensure you have Rust installed. If not, follow the [Rust installation guide](https://www.rust-lang.org/tools/install).
2. Clone the Flotsam repository: `git clone https://github.com/apple-fritter/flotsam.git`
3. Navigate to the project directory: `cd flotsam`
4. Build the project: `cargo build --release`

---

## Prerequisites
The following crates: regex and chrono.

The `regex` crate provides regular expression support, which is essential for parsing and manipulating the log files.  
The `chrono` crate is used for working with dates and timestamps, which can be helpful for generating the log file names.

---

## Usage
1. Run Flotsam with the log directory path as the argument:
```shell
$ flotsam ~/path/to/logs/
```
> If no argument is provided, Flotsam will prompt you to enter the log directory path.
2. Flotsam will recursively process the log files, counting user contributions and generating the TSV log.
3. The TSV log will be saved as `~/flotsamYYMMDDHHMMSS.log`, where `YYMMDDHHMMSS` represents the timestamp.
4. Enjoy the elegance and efficiency of Flotsam in your IRC moderation tasks!

---

## Flowchart
```
┌─ Start Program
│
├─ Load Log Directory
│   ├─ Validate Log Directory
│   ├─ Prompt for Log Directory if not provided
│   └─ Process Log Files
│       ├─ Read Log File
│       ├─ Validate Log File Format
│       ├─ Parse Log Entries
│       ├─ Filter Flagged Entries
│       ├─ Count Flagged Entries by User
│       └─ Generate TSV Log of User Counts
│
└─ End Program
```

---

## <a id="irc-meta"></a>🤪 IRC Meta
### <a id="fritterz"></a> [@apple-fritter](https://github.com/apple-fritter)'s IRC Repositories:

---

#### Driftwood Suite of IRC Analytics
###### Driftwood utilities
- [driftwood](https://github.com/apple-fritter/driftwood): A unified IRC log format definition. (Rust)
- [jetsam](https://github.com/apple-fritter/jetsam): Flag lines of driftwood formatted IRC logs for sanitization, moderation, or further review. (Rust)
- [scrimshaw](https://github.com/apple-fritter/scrimshaw): Create a quoteslist of any given user, from your driftwood formatted logs. (Rust)

##### Driftwood native logging plugins
- [weechat.driftwood](https://github.com/apple-fritter/weechat.driftwood): Natively log WeeChat messages in the driftwood standard. (Python)

---

#### heX-Chat
- [xchat.channel-moderation](https://github.com/apple-fritter/xchat.channel-moderation): Moderate an IRC channel. (Python)
- [doppelganger](https://github.com/apple-fritter/doppelganger): X-Chat mIRC imposter. Fingerprint subversion. (Python bundle)

---

#### IRCcloud
- [irccloud-to-weechat](https://github.com/apple-fritter/irccloud-to-weechat): Convert IRC logs from IRCcloud format to Weechat format. (Rust)
- [irccloud-to-xchat](https://github.com/apple-fritter/irccloud-to-xchat): Convert IRC logs from IRCcloud format to XChat format. (Rust)

---

#### WeeChat
- [weechat.driftwood](https://github.com/apple-fritter/weechat.driftwood): Natively log WeeChat messages in the driftwood standard. (Python)
- [weechat.ban-evasion-detection](https://github.com/apple-fritter/weechat.ban-evasion-detection): Detect and prevent ban evasion. (Python)
- [weechat.typo-aggregator](https://github.com/apple-fritter/weechat.typo-aggregator): Record misspelled words in a TSV (tab-separated values) file. (Python)
- [weechat.whois-aggregator](https://github.com/apple-fritter/weechat.whois-aggregator): Aggregate whois data in a rolling CSV file. (Python)
- [weechat.youtube-info](https://github.com/apple-fritter/weechat.youtube-info): Deprecated. Extract video information from a YouTube URL and post it back to the channel. (Python)
- [weechat.youtube-api](https://github.com/apple-fritter/weechat.youtube-api): Extract video information from a YouTube URL and post it back to the channel. (Python)

---

### IRC usage considerations
When working with any project involving IRC (Internet Relay Chat), it's important to keep the following considerations in mind to ensure a positive and respectful environment for all participants.

#### Philosophy of Use
Tailor your project's behavior and responses to align with the expected norms and conventions of IRC. Take into account the preferences and expectations of IRC users, ensuring that your project provides a seamless and familiar experience within the IRC ecosystem.

#### Foster a Positive and Inclusive Environment
Respect and adhere to the guidelines and policies of the IRC platform you are using. Familiarize yourself with the platform's rules regarding script usage, automation, and acceptable behavior. Comply with the platform's Terms of Service, and be mindful of any limitations or restrictions imposed by the platform. Strive to create an inclusive and welcoming environment where all users can engage respectfully and comfortably.

#### Respect the Rights and Dignity of Other Users
Maintain a polite and courteous demeanor in all interactions. Uphold the fundamental principles of respect, avoiding engagement in illegal, inappropriate, or offensive behavior. This includes refraining from using derogatory or inflammatory language, sharing explicit, triggering, or offensive content, engaging in harassment, or launching personal attacks. Obtain explicit consent before interacting with other users or sending automated responses. Respect the privacy of other users and avoid invading their personal space without their permission.

#### Respect the IRC Community and Channels
Avoid disrupting the normal flow of conversation within IRC channels. Ensure that your project's actions and responses do not cause unnecessary disruptions or inconvenience to other users. Implement mechanisms to prevent spamming or flooding the channel with excessive or irrelevant messages. Handle errors gracefully, preventing unintended behavior or disruptions to the IRC platform or the experiences of other users.

#### Ensure Compatibility
Consider the potential variations in behavior across different IRC platforms and clients. While aiming for compatibility, be aware that certain functionalities may not be available or consistent across all platforms. Test your project on multiple IRC platforms and clients to ensure compatibility and provide the best possible experience for users.

---

## Contributing
Contributions are welcome! If you'd like to contribute, please follow these steps:
1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them.
4. Push your changes to your forked repository.
5. Submit a pull request to the main repository.

---

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

---

## License
This project is licensed under the [MIT License](LICENSE).
