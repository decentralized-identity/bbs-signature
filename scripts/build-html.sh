# Convert markdown to XML and HTML versions
docker run -v `pwd`:/data danielfett/markdown2rfc spec.md || exit 1

# Delete XML version
rm bbs-signatures-00.xml

# Rename the HTML version for hosting with GH pages
mv bbs-signatures-00.html index.html