function fdiff -d "Fancy diff"
  diff -u $argv | diff-so-fancy | bat
end
