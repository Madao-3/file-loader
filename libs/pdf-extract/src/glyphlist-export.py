f = open("glyphlist-extended.txt")
lines = f.readlines()
import re
glyphlist = []
for l in lines:
    if l[0] == '#' or l[0] == '\n':
        continue
    name, code = re.split('[; ]+', l)[0:2]
    glyphlist.append((name,int(code,16)))
glyphlist.sort()
print "/* Autogenerated from https://github.com/michal-h21/htfgen/commits/master/glyphlist-extended.txt */"
print "pub fn name_to_unicode(name: &str) -> Option<u16> {"
print "    let names = ["
print ",\n".join('(\"%s\", 0x%04x)' % (g[0], g[1]) for g in glyphlist)
print "    ];"
print "    let result = names.binary_search_by_key(&name, |&(name,code)| &name);"
print "    result.ok().map(|indx| names[indx].1)"
print "}"
