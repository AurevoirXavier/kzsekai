package parser

import (
    "encoding/base64"
    "github.com/PuerkitoBio/goquery"
    "regexp"
    "sexy/engine"
    "sexy/fetcher"
    "strconv"
    "strings"
)

type Post struct {
    Categories []string
    Id         string
    Models     []string
    SiteId     string
    Tags       []string
    Title      string

    Parts []string
}

func decode(b64Code string) string {
    var (
        salt         = "fLgwk8@9a*ag_)eq&#I^a6h8h#13"
        saltLen      = len(salt)
        b64Decode, _ = base64.StdEncoding.DecodeString(b64Code)
        s            = string(b64Decode)

        vi []int
        vs []string
    )

    vi = nil
    for i := 0; i < len(s)-1; i += 2 {
        var x, _ = strconv.ParseInt(s[i:i+2], 16, 0)
        vi = append(vi, int(x))
    }

    s = ""
    for _, c := range vi {
        s += string(c)
    }
    vs = strings.Split(s, " ")
    for i := 0; i < len(vs); i += 1 {
        for len(vs[i]) < 8 {
            vs[i] = "0" + vs[i]
        }
    }

    for i := 0; i < len(vs); i += 1 {
        var x, _ = strconv.ParseInt(vs[i], 2, 0)
        vs[i] = string(int(x))
    }

    s = ""
    for i := 0; i < len(vs); i += 1 {
        var j = i % saltLen
        s += string(int(vs[i][0]) ^ int(salt[j]))
    }

    return s
}

func parseScript(script string) []string {
    var (
        re    *regexp.Regexp
        parts []string
    )

    re = regexp.MustCompile(`data=JSON\.parse\(atob\("(.+?)"\)\)`)
    var (
        b64Code      = re.FindStringSubmatch(script)[1]
        b64Decode, _ = base64.StdEncoding.DecodeString(b64Code)
    )

    re = regexp.MustCompile(`vid=(.+?)"`)
    for _, code := range re.FindAllStringSubmatch(string(b64Decode), -1) {
        parts = append(parts, decode(code[1]))
    }

    return parts
}

func ParsePost(doc *goquery.Document, fc *fetcher.Fetcher) engine.ParseResult {
    var post = Post{}

    doc.Find(`.video-countext-categories a`).Each(func(_ int, s *goquery.Selection) {
        var tltle, _ = s.Attr("tltle")
        post.Categories = append(post.Categories, tltle)
    })

    var (
        postUrl, _ = doc.Find(`head > link:nth-child(6)`).Attr("href")
        splitUrl   = strings.Split(postUrl, "/")
    )
    post.Id = splitUrl[len(splitUrl)-1]
    post.SiteId = splitUrl[len(splitUrl)-2]

    post.Title = doc.Find(`.video-title h1`).Text()

    doc.Find(`.video-countext-tags a`).Each(func(_ int, s *goquery.Selection) {
        var title, _ = s.Attr("title")
        post.Tags = append(post.Tags, title)
    })

    doc.Find(`.video-box-model-name`).Each(func(_ int, s *goquery.Selection) {
        post.Models = append(post.Models, s.Text())
    })

    post.Parts = parseScript(doc.Find(`#down_file > script:nth-child(2)`).Text())

    return engine.ParseResult{Item: post}
}
