#!/bin/zsh
echo $1
case $1 in
  "now")
    touch src/problems/$2.rs
    sed "s/FILE/$2/g" < src/.problemsTemplate.txt > src/problems.rs
    ;;
  "commit")
    git add src/problems/$2.rs && git commit -m "Finished $2"
    ;;
  *)
    echo "Syntax wrong"
    ;;
esac
