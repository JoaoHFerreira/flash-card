import psycopg2
import psycopg2.pool
import scrapy
import json
from scrapy.crawler import CrawlerProcess


class FetchLinksSpiderCommand:
    def execute(self):
        process = CrawlerProcess(
            {
                "USER_AGENT": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"
            }
        )

        process.crawl(LinksSpider)
        process.start()


class LinksSpider(scrapy.Spider):
    name = "wikipedia"

    #TODO Starting URL must be a parameter in the future
    start_urls = ["https://de.wikipedia.org/wiki/Deutschland"]
    links_collected = set()

    def parse(self, response):
        new_links = self._get_valid_links(response)
        self.links_collected.update(new_links)
        if len(self.links_collected) < 1000:
            # Continue scraping
            for link in new_links:
                yield scrapy.Request(link, callback=self.parse)
        else:
            self.save_links()

    def _get_valid_links(self, response):
        links = response.xpath("//div[@id='bodyContent']//a/@href").getall()
        valid_links = [
            response.urljoin(link)
            for link in links
            if link.startswith("/wiki/") and ":" not in link
        ]
        return valid_links

    def save_links(self):
        links_list = list(self.links_collected)[:1000]  # Ensure we only save 1000 links
        # TODO list name will be generated automaticly depending on the choosed language
        # TODO Instead of saving in a json list, save in a database, will have url as id and language
        with open("german_list.json", "w") as f:
            json.dump(links_list, f)
