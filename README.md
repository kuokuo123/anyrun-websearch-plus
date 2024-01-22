# Anyrun-Websearch-Plus
An [anyrun](https://github.com/Kirottu/anyrun) plugin to search the web through customized search engines. Different from the original [ websearch ](https://github.com/Kirottu/anyrun/tree/master/plugins/websearch) plugin, this one allows for setting prefixes to call for a specified search engine.

## Usage

Enter your search term with a prefix to call for the search engine. If more than two search engines have the same prefix, select the search action you want with arrow keys.

![Demo Video](https://github.com/kuokuo123/anyrun-websearch-plus/raw/main/asset/demo.mkv?raw=true)

## Configuration

An example config.

```ron
// <Anyrun config dir>/websearch_plus.ron

Config(

  // The main prefix calls for websearch; a secondary prefix set in the Custom engine block calls for a specified search engine.
  // for example, in this config "?d Tux the penguin" calls DuckDuckGo to search for Tux the penguin, "?awiki " calls Archwiki, "?" calls Google. You can also depend wholely on secondary prefixes by setting the main prefix to "".

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

