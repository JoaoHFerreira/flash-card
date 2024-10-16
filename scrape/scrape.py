import psycopg2
import psycopg2.pool
import scrapy
import json
from scrapy.crawler import CrawlerProcess


class LinksSpider(scrapy.Spider):
    name = "wikipedia"
    start_urls = ["https://en.wikipedia.org/wiki/Web_scraping"]
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
            if link.startswith('/wiki/') and ':' not in link
        ]
        return valid_links

    def save_links(self):
        links_list = list(self.links_collected)[:1000]  # Ensure we only save 1000 links
        with open('links_list.json', 'w') as f:
            json.dump(links_list, f)


class WikipediaSpider(scrapy.Spider):
    name = "wikipedia"

    def __init__(self, url, pg_pool, *args, **kwargs):
        super(WikipediaSpider, self).__init__(*args, **kwargs)
        self.start_urls = [url]
        self.pg_pool = pg_pool

    def parse(self, response):
        article_text = response.xpath("//div[@id='bodyContent']//p//text()").getall()
        article_text = ' '.join(article_text).strip()
        
    
        self._save_to_postgres(
            response.url,
            article_text,
        )

    def _save_to_postgres(self, url, article_text):
        with pool.getconn() as conn:
            with conn.cursor() as cur:
                try:
                    cur.execute(
                        """
                        INSERT INTO wikipedia_articles (id, content)
                        VALUES (%s, %s)
                        ON CONFLICT (id) DO UPDATE
                            SET content = EXCLUDED.content;
                        """,
                        (url, article_text)
                    )
                    conn.commit()
                except Exception as e:
                    conn.rollback()
                    print(f"Error saving article {url}: {e}")


if __name__ == "__main__":
    process = CrawlerProcess({
        'USER_AGENT': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
    })
    

    pool = psycopg2.pool.SimpleConnectionPool(
        minconn=1,  # Minimum number of connections
        maxconn=10,  # Maximum number of connections
        dbname="flash_card_db",
        user="user",
        password="pass",
        host="db",
        port="5432"
    )

    with open('new.json', 'r') as f:
        links = json.load(f)

    for wiki in links:
        process.crawl(WikipediaSpider, url=wiki, pg_pool=pool)
    
    process.start()
