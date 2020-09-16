# Github

* From your project repository, bring in the changes and test.

```
git fetch origin
git checkout -b docu-entwurf origin/docu-entwurf
git merge master
```

* Merge the changes and update on GitHub.

```
git checkout master
git merge --no-ff docu-entwurf
git push origin master
```
