package fetcher

import (
    "crypto/tls"
    "fmt"
    bypasser "github.com/AurevoirXavier/cloudflare-bypasser-go"
    "github.com/PuerkitoBio/goquery"
    "log"
    "net/http"
    "net/url"
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
