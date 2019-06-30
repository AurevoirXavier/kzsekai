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
    Free map[string]map[string]string
    HD   map[string]map[string]string
}

type Post struct {
    Id     string
    SiteId string
    Title  string

    Categories []string
    Models     []string
    Tags       []string

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
        parts.HD = make(map[string]map[string]string)

        for k1, v := range hds.(map[string]interface{}) {
            parts.HD[k1] = make(map[string]string)

            switch v := v.(type) {
            case string:
                parts.HD[k1]["A"] = v
            case map[string]interface{}:
                for k2, v := range v {
                    parts.HD[k1][k2] = v.(string)
                }
            }
        }

        delete(raw, "HD")
    }

    if len(raw) != 0 {
        var format = "%s"

        re = regexp.MustCompile(`vid=(.+)`)
        parts.Free = make(map[string]map[string]string)

        for k1, v := range raw {
            switch k1 {
            case "asc":
                k1 = "asianclub"
                format = "https://asianclub.tv/f/%s"
            case "VS":
                k1 = "verystream"
                format = "https://verystream.com/stream/%s"
            case "VO":
                k1 = "vidoza"
                format = "https://vidoza.net/%s.html"
            }

            parts.Free[k1] = make(map[string]string)

            switch v := v.(type) {
            case string:
                parts.Free[k1]["A"] = fmt.Sprintf(format, decode2(decode1(re.FindStringSubmatch(v)[1])))
            case map[string]interface{}:
                for k2, v := range v {
                    parts.Free[k1][k2] = fmt.Sprintf(format, decode2(decode1(re.FindStringSubmatch(v.(string))[1])))
                }
            }
        }
    }

    return parts
}

func ParsePost(doc *goquery.Document) engine.ParseResult {
    var post = Post{}

    var (
        postUrl, _ = doc.Find(`head > link:nth-child(6)`).Attr("href")
        splitUrl   = strings.Split(postUrl, "/")
    )
    post.Id = splitUrl[len(splitUrl)-1]
    post.SiteId = splitUrl[len(splitUrl)-2]
    post.Title = doc.Find(`.video-title h1`).Text()

    doc.Find(`.video-countext-categories a`).Each(func(_ int, s *goquery.Selection) {
        var tltle, _ = s.Attr("tltle")
        post.Categories = append(post.Categories, tltle)
    })
    doc.Find(`.video-box-model-name`).Each(func(_ int, s *goquery.Selection) {
        post.Models = append(post.Models, s.Text())
    })
    doc.Find(`.video-countext-tags a`).Each(func(_ int, s *goquery.Selection) {
        var title, _ = s.Attr("title")
        post.Tags = append(post.Tags, title)
    })

    post.Parts = parseScript(doc.Find(`#down_file > script:nth-child(2)`).Text())

    return engine.ParseResult{Item: post}
}
