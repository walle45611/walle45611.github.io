/* global hexo */
'use strict';

const { generateRssFeed, generateAtomFeed } = require('feedsmith');
const {
  encodeURL,
  full_url_for,
  gravatar,
  stripHTML,
  unescapeHTML,
  truncate
} = require('hexo-util');

function normalizeWhitespace(value) {
  return value.replace(/\s+/g, ' ').trim();
}

function toPlainText(value, limit) {
  if (!value) return '';

  let text = stripHTML(String(value));
  text = unescapeHTML(text);
  text = text.replace(/[\x00-\x1F\x7F]/g, ' ');
  text = normalizeWhitespace(text);

  if (limit) text = truncate(text, limit);

  return text;
}

function composePosts(posts, feedConfig) {
  const { limit, order_by } = feedConfig;

  let processedPosts = posts.sort(order_by || '-date');
  processedPosts = processedPosts.filter(post => post.draft !== true);

  if (limit) processedPosts = processedPosts.limit(limit);

  return processedPosts;
}

function composeFeed(config, path, context, posts) {
  const { feed: feedConfig, url: urlConfig, email } = config;
  const { icon: iconConfig, hub } = feedConfig;

  let url = urlConfig;
  if (url[url.length - 1] !== '/') url += '/';

  let icon = '';
  if (iconConfig) icon = full_url_for.call(context, iconConfig);
  else if (email) icon = gravatar(email);

  const feedUrl = full_url_for.call(context, path);
  const currentYear = new Date().getFullYear();

  return {
    title: config.title,
    description: config.subtitle || config.description,
    url,
    feedUrl,
    icon,
    hub,
    language: config.language,
    author: { name: config.author, email: config.email },
    copyright: config.author && `All rights reserved ${currentYear}, ${config.author}`,
    updated: posts.first().updated ? posts.first().updated.toDate() : posts.first().date.toDate()
  };
}

function composeFeedLinks(feedUrl, hub, type) {
  const links = [{ href: encodeURL(feedUrl), rel: 'self', type }];
  if (hub) links.push({ href: encodeURL(hub), rel: 'hub' });
  return links;
}

function composeItemDescription(post, feedConfig) {
  const { content_limit, content_limit_delim } = feedConfig;

  let description = '';

  if (post.description) {
    description = post.description;
  } else if (post.intro) {
    description = post.intro;
  } else if (post.excerpt) {
    description = post.excerpt;
  } else if (post.content) {
    const shortContent = post.content.substring(0, content_limit || 140);
    if (content_limit_delim) {
      const delimPos = shortContent.lastIndexOf(content_limit_delim);
      description = delimPos > -1 ? shortContent.substring(0, delimPos) : shortContent;
    } else {
      description = shortContent;
    }
  }

  return toPlainText(description, content_limit || 140);
}

function composeItemContent(post, feedConfig) {
  const { content } = feedConfig;

  if (content && post.content) {
    return post.content.replace(/[\x00-\x1F\x7F]/g, '');
  }
  return '';
}

function composeItemCategories(post) {
  const items = [
    ...post.categories ? post.categories.toArray() : [],
    ...post.tags ? post.tags.toArray() : []
  ];
  return items.map(item => ({ name: item.name, domain: item.permalink }));
}

function composeItem(post, feedConfig, context) {
  return {
    title: post.title,
    link: encodeURL(full_url_for.call(context, post.permalink)),
    description: composeItemDescription(post, feedConfig),
    published: post.date.toDate(),
    updated: post.updated ? post.updated.toDate() : post.date.toDate(),
    content: composeItemContent(post, feedConfig),
    enclosures: post.image && [{ url: full_url_for.call(context, post.image) }],
    categories: composeItemCategories(post)
  };
}

function composeRssItem(feed, item) {
  return {
    title: item.title,
    link: item.link,
    guid: item.link,
    description: item.description,
    pubDate: item.published,
    authors: [feed.author],
    content: { encoded: item.content },
    enclosures: item.enclosures,
    categories: item.categories
  };
}

function composeAtomEntry(feed, item) {
  const entryLinks = [
    { href: item.link },
    ...(item.enclosures || []).map(enclosure => ({ href: enclosure.url, rel: 'enclosure' }))
  ];

  return {
    title: item.title,
    id: item.link,
    links: entryLinks,
    summary: item.description,
    content: item.content,
    published: item.published,
    updated: item.updated || item.published,
    authors: feed.author.name && [feed.author],
    categories: item.categories.map(cat => ({ term: cat.name, scheme: cat.domain }))
  };
}

function generateRss(feed, items) {
  const links = composeFeedLinks(feed.feedUrl, feed.hub, 'application/rss+xml');

  return generateRssFeed({
    title: feed.title,
    description: feed.description,
    link: encodeURL(feed.url),
    language: feed.language,
    copyright: feed.copyright,
    generator: 'Hexo',
    lastBuildDate: feed.updated,
    image: feed.icon && {
      url: feed.icon,
      title: feed.title,
      link: encodeURL(feed.url)
    },
    atom: { links },
    items: items.map(item => composeRssItem(feed, item))
  }, { lenient: true });
}

function generateAtom(feed, items) {
  const links = [
    { href: encodeURL(feed.url), rel: 'alternate' },
    ...composeFeedLinks(feed.feedUrl, feed.hub)
  ];

  return generateAtomFeed({
    title: feed.title,
    id: encodeURL(feed.url),
    subtitle: feed.description,
    updated: feed.updated,
    links,
    generator: { text: 'Hexo', uri: 'https://hexo.io/' },
    icon: feed.icon,
    rights: feed.copyright,
    authors: feed.author.name && [feed.author],
    entries: items.map(item => composeAtomEntry(feed, item)),
    language: feed.language
  }, { lenient: true });
}

function generateFeed(locals, type, path) {
  const { config } = this;
  const { feed: feedConfig } = config;
  const posts = composePosts(locals.posts, feedConfig);

  if (posts.length <= 0) {
    feedConfig.autodiscovery = false;
    return;
  }

  const feed = composeFeed(config, path, this, posts);
  const items = posts.toArray().map(post => composeItem(post, feedConfig, this));

  const data = type === 'rss2' ? generateRss(feed, items) : generateAtom(feed, items);

  return {
    path,
    data
  };
}

const { feed: feedConfig } = hexo.config;

if (feedConfig && feedConfig.enable !== false) {
  if (typeof feedConfig.type === 'string') {
    hexo.extend.generator.register(feedConfig.type, locals => {
      return generateFeed.call(hexo, locals, feedConfig.type, feedConfig.path);
    });
  } else if (Array.isArray(feedConfig.type)) {
    for (const feedType of feedConfig.type) {
      hexo.extend.generator.register(feedType, locals => {
        return generateFeed.call(hexo, locals, feedType, feedConfig.path[feedConfig.type.indexOf(feedType)]);
      });
    }
  }
}
