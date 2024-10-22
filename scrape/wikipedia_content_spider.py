import psycopg2
import psycopg2.pool
import scrapy
import json
import time
from scrapy.spidermiddlewares.httperror import HttpError
from twisted.internet.error import DNSLookupError, TCPTimedOutError
from scrapy.crawler import CrawlerProcess
import scrapy


class WikipediaContentSpiderCommand:
    def execute(self):
        pool = psycopg2.pool.ThreadedConnectionPool(
            minconn=1,
            maxconn=5,  # Reduced number of connections
            dbname="flash_card_db",
            user="user",
            password="pass",
            host="db",
            port="5432",
        )

        # TODO Create download flag logic
        # TODO Logic to compare existent before, only download not flagged
        # TODO make it also dynamic, should be consulting a database ideally, based on last insert
        with open("german_list.json", "r") as f:
            urls = json.load(f)

        process = CrawlerProcess(
            {
                "USER_AGENT": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"
            }
        )

        process.crawl(WikipediaSpider, urls=urls, pg_pool=pool)
        process.start()

        pool.closeall()
        # TODO add a last method here, to insert the column country, but only for those that dont have..


class WikipediaSpider(scrapy.Spider):
    name = 'wikipedia'
    allowed_domains = ['wikipedia.org']
    custom_settings = {
        'ROBOTSTXT_OBEY': True,
        'DOWNLOAD_DELAY': 1,  # 1 second delay between requests
        'CONCURRENT_REQUESTS_PER_DOMAIN': 1,  # Only one concurrent request per domain
        'USER_AGENT': 'WikipediaResearchBot (https://www.example.com/bot-info)',  # Replace with your info
    }

    def __init__(self, urls, pg_pool, *args, **kwargs):
        super(WikipediaSpider, self).__init__(*args, **kwargs)
        self.start_urls = urls
        self.pg_pool = pg_pool

    def start_requests(self):
        for url in self.start_urls:
            yield scrapy.Request(url, callback=self.parse, errback=self.errback_httpbin)

    def parse(self, response):
        article_text = response.xpath("//div[@id='bodyContent']//p//text()").getall()
        article_text = " ".join(article_text).strip()
        self._save_to_postgres(response.url, article_text)

    def errback_httpbin(self, failure):
        if failure.check(HttpError):
            response = failure.value.response
            self.logger.error(f'HttpError on {response.url}')
        elif failure.check(DNSLookupError):
            request = failure.request
            self.logger.error(f'DNSLookupError on {request.url}')
        elif failure.check(TimeoutError, TCPTimedOutError):
            request = failure.request
            self.logger.error(f'TimeoutError on {request.url}')

    def _save_to_postgres(self, url, article_text):
        max_retries = 3
        retry_delay = 1  # seconds
        
        for attempt in range(max_retries):
            conn = None
            try:
                conn = self.pg_pool.getconn()
                with conn.cursor() as cur:
                    cur.execute(
                        """
                        INSERT INTO wikipedia_articles (id, content)
                        VALUES (%s, %s)
                        ON CONFLICT (id) DO UPDATE
                        SET content = EXCLUDED.content;
                        """,
                        (url, article_text),
                    )
                conn.commit()
                self.logger.info(f"Successfully saved article: {url}")
                return
            except psycopg2.Error as e:
                conn.rollback()
                self.logger.error(f"Database error saving article {url} (attempt {attempt + 1}/{max_retries}): {e}")
                if attempt < max_retries - 1:
                    time.sleep(retry_delay)
            except Exception as e:
                self.logger.error(f"Unexpected error saving article {url}: {e}")
            finally:
                if conn:
                    self.pg_pool.putconn(conn)
        
        self.logger.error(f"Failed to save article after {max_retries} attempts: {url}")
