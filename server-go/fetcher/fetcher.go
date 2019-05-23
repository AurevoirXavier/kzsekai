package fetcher

import (
    "crypto/tls"
    "fmt"
    bypasser "github.com/AurevoirXavier/cloudflare-bypasser-go"
    "github.com/PuerkitoBio/goquery"
    "log"
    "net/http"
    "net/url"
    "strconv"
)

type Fetcher struct {
    *http.Client

    UserAgent string
}

func (fc *Fetcher) SetProxy(proxyUrl string) {
    var (
        u, _      = url.Parse(proxyUrl)
        transport = http.Transport{}
    )
    transport.Proxy = http.ProxyURL(u)
    transport.TLSClientConfig = &tls.Config{InsecureSkipVerify: true}
    fc.Transport = &transport
}

func (fc *Fetcher) Bypass(host string) {
    log.Println("trying to bypass,", host)

    var (
        client          = bypasser.NewBypasser(fc.Client)
        req, _          = http.NewRequest("GET", host, nil)
        userAgent, _, _ = client.Bypass(req, 0)
    )
    fc.UserAgent = userAgent
}

func (fc *Fetcher) FetchDoc(req *http.Request) (*goquery.Document, error) {
    //log.Println("fetching", req.URL)

    var resp, e = fc.Do(req)
    if e != nil {
        return nil, e
    }
    defer resp.Body.Close()

    if resp.StatusCode != http.StatusOK {
        return nil, fmt.Errorf("unexpected status code, %d", resp.StatusCode)
    }

    return goquery.NewDocumentFromReader(resp.Body)
}

func (fc *Fetcher) GetLastPage(pageUrl string, selector string) uint16 {
    log.Println("getting last page from,", pageUrl)

    var req, _ = http.NewRequest("GET", pageUrl, nil)
    req.Header.Set("User-Agent", fc.UserAgent)

    var (
        doc, _       = fc.FetchDoc(req)
        lastPageATag = doc.Find(selector).Text()
        lastPage, _  = strconv.Atoi(lastPageATag)
    )
    return uint16(lastPage)
}
