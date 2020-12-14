import htmlparser
import xmltree
import strtabs
import strutils



var file = open("../html/t.txt",fmRead)

proc writeLinks(html_string: cstring): void {.stdcall,exportc,dynlib.} =
    var html = parseHtml($html_string)

    for a in html.findAll("a"):
        if a.attrs.hasKey "href":
            var href = a.attrs["href"]

            if "/perl/chessgame" in href:
                add(href,"\n")
                file.write(href)
