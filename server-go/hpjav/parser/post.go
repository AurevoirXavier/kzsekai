package parser

import (
    "encoding/base64"
    "encoding/json"
    "fmt"
    "github.com/PuerkitoBio/goquery"
    "regexp"
    "sexy/engine"
    "strconv"
    "strings"
)

type Parts struct {
    Free map[string]string
    HD   map[string]string
}

type Post struct {
    Categories []string
    Id         string
    Models     []string
    SiteId     string
    Tags       []string
    Title      string

    Parts Parts
}

func decode1(b64Code string) string {
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

func decode2(code string) string {
    var b64Code = ""
    for i := len(code); i > 1; i -= 2 {
        var x, _ = strconv.ParseInt(code[i-1:i]+code[i-2:i-1], 16, 0)
        b64Code += string(int(x))
    }

    var b64Decode, _ = base64.StdEncoding.DecodeString(b64Code)
    return string(b64Decode)
}

func parseScript(script string) Parts {
    var (
        re    *regexp.Regexp
        parts Parts
    )

    re = regexp.MustCompile(`data=JSON\.parse\(atob\("(.+?)"\)\)`)
    var (
        b64Code      = re.FindStringSubmatch(script)[1]
        b64Decode, _ = base64.StdEncoding.DecodeString(b64Code)

        raw = make(map[string]interface{})
        _   = json.Unmarshal(b64Decode, &raw)
    )

    if hds, ok := raw["HD"]; ok {
        parts.HD = make(map[string]string)
        for k, v := range hds.(map[string]interface{}) {
            fmt.Printf("%#v\n", v)
            switch v := v.(type) {
            case string:
                parts.HD[k] = v
            case map[string]interface{}:
                fmt.Println(v)
            }
        }

        delete(raw, "HD")
    }

    if len(raw) != 0 {
        re = regexp.MustCompile(`host=(.+?)&vid=(.+)`)
        parts.Free = make(map[string]string)

        for _, v := range raw {
            switch v.(type) {
            case string:
                var (
                    matched = re.FindStringSubmatch(v.(string))
                    k       = matched[1]
                    v       = decode2(decode1(matched[2]))
                )

                switch k {
                case "asianclub":
                    v = fmt.Sprintf("https://asianclub.tv/f/%s", v)
                case "verystream":
                    v = fmt.Sprintf("https://verystream.com/stream/%s", v)
                case "vidoza":
                    v = fmt.Sprintf("https://vidoza.net/%s.html", v)
                }

                parts.Free[k] = v
            case map[string]interface{}:
                fmt.Println(v)
            }
        }
    }

    return parts
}

func ParsePost(doc *goquery.Document) engine.ParseResult {
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
