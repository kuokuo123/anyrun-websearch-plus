# Anyrun-Websearch-Plus
An [anyrun](https://github.com/Kirottu/anyrun) plugin to call for web search through customized search engines. Different from the original [ websearch ](https://github.com/Kirottu/anyrun/tree/master/plugins/websearch) plugin, this one allow for setting prefixes to call for specified search engines.

## Usage

Enter your search term with a prefix to call for the search engine. If more than two search engines have the same prefix, select the search action you want with arrow keys.

![Demo Video](https://github.com/kuokuo123/anyrun-websearch-plus/raw/main/asset/demo.mkv?raw=true)

## Configuration

Default config

```ron
// <Anyrun config dir>/websearch_plus.ron

Config(

  // The prefix below calls for websearch; a secondary prefix set in the Custom engine block calls for a specified search engine.
  // for example, in this config "?d Tux the penguin" calls DuckDuckGo to search for Tux the penguin, "?awiki " calls Archwiki, "?" calls Google.

  prefix: "?",

  // Options: Google, Custom
  // The default google engine doesn't have a secondary prefix. If you need a prefix for google search, set a customized google engine and remove the default one.
  // NOTE: `{}` is replaced by the search query and `https://` is automatically added in front.

  engines: [

    // Example config:

    Custom(
      name: "DuckDuckGo",
      url: "duckduckgo.com/?q={}",
      secondary_prefix: "d ",
    ),

    Custom(
      name: "Arch Package Repository",
      url: "archlinux.org/packages/?q={}",
      secondary_prefix: "apkg ",
    ),

    Custom(
      name: "Archwiki",
      url: "wiki.archlinux.org/index.php?search={}",
      secondary_prefix: "awiki ",
    ),

    Custom(
      name: "Arch User Repository",
      url: "aur.archlinux.org/packages?K={}",
      secondary_prefix: "aur ",
    ),

    Custom(
      name: "Github",
      url: "github.com/search?q={}",
      secondary_prefix: "gh ",
    ),

    Custom(
      name: "PTT",
      url: "www.pttweb.cc/ptt-search#gsc.q={}",
      secondary_prefix: "ptt ",
    ),

    Custom(
      name: "Reddit",
      url: "www.reddit.com/search/?q={}",
      secondary_prefix: "rd ",
    ),

    Custom(
      name: "Youtube",
      url: "www.youtube.com/results?search_query={}",
      secondary_prefix: "yt ",
    ),

    Google,
  ],
     
)
```

