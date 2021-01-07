require File.dirname(__FILE__) + '/lib/custom_redcarpet.rb'

# Activate and configure extensions
# https://middlemanapp.com/advanced/configuration/#configuring-extensions

activate :autoprefixer do |prefix|
  prefix.browsers = "last 2 versions"
end

# Layouts
# https://middlemanapp.com/basics/layouts/

# Per-page layout changes
page '/*.xml', layout: false
page '/*.json', layout: false
page '/*.txt', layout: false

# With alternative layout
# page '/path/to/file.html', layout: 'other_layout'

# Proxy pages
# https://middlemanapp.com/advanced/dynamic-pages/

# proxy(
#   '/this-page-has-no-template.html',
#   '/template-file.html',
#   locals: {
#     which_fake_page: 'Rendering a fake page with a local variable'
#   },
# )

# Helpers
# Methods defined in the helpers block are available in templates
# https://middlemanapp.com/basics/helper-methods/

# helpers do
#   def some_helper
#     'Helping'
#   end
# end

# Build-specific configuration
# https://middlemanapp.com/advanced/configuration/#environment-specific-settings

# configure :build do
#   activate :minify_css
#   activate :minify_javascript
# end

# Per-page layout changes
page '/*.xml', layout: false
page '/*.json', layout: false
page '/*.txt', layout: false

# Markdown setting
# set :markdown,
#       tables:             true,
#       autolink:           true,
#       gh_blockcode:       true,
#       fenced_code_blocks: true,
#       with_toc_data:      false

# set :markdown,
#     fenced_code_blocks: true,
#     smartypants: true,
#     with_toc_data: true,
#     tables: true,
#     autolink: true,
#     gh_blockcode: true
#     renderer: CustomRenderer

set :markdown,
      tables:             true,
      autolink:           true,
      gh_blockcode:       true,
      fenced_code_blocks: true,
      with_toc_data:      false,
      renderer:           CustomRenderer

set :markdown_engine, :redcarpet

activate :syntax

# Use relative URLs
activate :relative_assets
set :relative_links, true
set :strip_index_file, false

set :encoding, 'utf-8'

set :css_dir, 'stylesheets'
set :js_dir, 'sourse/javascripts'
set :images_dir, 'images'
