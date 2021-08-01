#!/usr/bin/env python
# -*- coding: utf-8 -*- #
from __future__ import unicode_literals
import os

AUTHOR = u'RMOTW Contributors'
SITENAME = u"Rust Module of the Week"
#SITEURL = 'https://this-week-in-rust.org'
SITEURL = "http://localhost:8000"
SOURCE_URL = 'https://github.com/slyons/rust-module-of-the-week'

THEME = 'themes/pelican-rusted-theme'

PLUGIN_PATHS = ["plugins"]
PLUGINS = ['assets', 'neighbors']

TIMEZONE = 'America/New_York'

DEFAULT_LANG = u'en'

FEED_DOMAIN = SITEURL
FEED_ALL_ATOM = 'atom.xml'
FEED_ALL_RSS = 'rss.xml'
FEED_MAX_ITEMS = 4
CATEGORY_FEED_ATOM = 'categories/%s/atom.xml'
CATEGORY_FEED_RSS = 'categories/%s/rss.xml'

DEFAULT_PAGINATION = 10

ARTICLE_URL = 'blog/{date:%Y}/{date:%m}/{date:%d}/{slug}/'
ARTICLE_SAVE_AS = 'blog/{date:%Y}/{date:%m}/{date:%d}/{slug}/index.html'

ARCHIVES_SAVE_AS = 'blog/archives/index.html'

CATEGORY_URL = 'categories/{slug}/'
CATEGORY_SAVE_AS = 'categories/{slug}/index.html'

LANDING_PAGE_ABOUT = {
        "title": "A weekly dive into commonly used modules in the Rust ecosystem.",
        "details": """
A weekly dive into commonly used modules in the Rust ecosystem, with story flavor! Follow a cast of characters as they 
try to solve their problems using the most commonly used modules available to Rust.
"""
}

MD_EXTENSIONS = ['headerid',
    'codehilite(css_class=highlight)',
    'extra', 
    'markdown.extensions.fenced_code',
    'markdown.extensions.admonition',
    ]
