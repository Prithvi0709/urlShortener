import urllib3
import random
baseurl = "0.0.0.0:8080"

listt = [ 'http://www.google.com', 'http://www.yahoo.com', 'http://www.bing.com' , "http://www.facebook.com", "http://www.twitter.com", "http://www.instagram.com", "http://www.linkedin.com", "http://www.youtube.com", "http://www.reddit.com", "http://www.wikipedia.org", "http://www.amazon.com", "http://www.ebay.com", "http://www.paypal.com", "http://www.craigslist.org", "http://www.stackoverflow.com", "http://www.wordpress.com", "http://www.tumblr.com", "http://www.live.com", "http://www.apple.com", "http://www.microsoft.com", "http://www.adobe.com", "http://www.orkut.com", "http://www.imdb.com", "http://www.qq.com", "http://www.blogger.com", "http://www.yandex.ru", "http://www.netflix.com", "http://www.office.com", "http://www.wordpress.org", "http://www.github.com", "http://www.espn.com", "http://www.chase.com", "http://www.cnn.com", "http://www.dropbox.com", "http://www.etsy.com", "http://www.adobe.com", "http://www.walmart.com", "http://www.cnet.com", "http://www.nytimes.com", "http://www.foxnews.com", "http://www.wikia.com", "http://www.cbs.com", "http://www.bbc.co.uk", "http://www.salesforce.com", "http://www.foxsports.com", "http://www.whatsapp.com", "http://www.cisco.com", "http://www.dailymotion.com", "http://www.att.com", "http://www.booking.com", "http://www.buzzfeed.com", "http://www.imgur.com", "http://www.zillow.com", "http://www.indeed.com", "http://www.stackexchange.com", "http://www.dell.com", "http://www.irs.gov", "http://www.usps.com", "http://www.ups.com", "http://www.alibaba.com", "http://www.nike.com", "http://www.target.com", "http://www.tripadvisor.com", "http://www.vice.com", "http://www.abc.net.au", "http://www.nbc.com"]

print(f"Starting test, hitting {len(listt)} websites.")

http = urllib3.PoolManager()
for count, website in enumerate(listt):
    r = http.request('GET', f"{baseurl}/shorten?url={website}&translation_type={random.randint(1, 4)}")
    for i in range(count + 1) :
        j = http.request('GET',  f"{baseurl}/{r.data.decode('utf-8')}" , redirect = False).data