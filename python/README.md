# Learning [Your language or framework] REST API Project


  This is a basic Go project created for learning purposes. The project emphasizes making REST API calls and implementing middleware for a simple web application.
 
## Getting Running

### Installation/Prerequisites

  We recommend using the latest version of Python. Flask supports Python 3.8 and newer. Specifically, this API was built with Python 3.11.
  You will also want to create a Python virtual environment for easy management of packages and dependencies in both development and production. You can do so like this:
```
  mkdir myproject
  cd myproject
  python3 -m venv .venv
  . .venv/bin/activate
  pip install flask
```

That's it!

### Running Project

  To run the server, simply navigate to the directory where your "app.py" file, and run:
```
  flask run --debug -h localhost -p 8000

```
The debug flag is optional, but will make the process of debugging the API easier (obviously)

### Language Project layout resources

  *** 1-3 links providing the correct project layout for your language.
  [PEPS 8 Style Guide](https://peps.python.org/pep-0008/)

### Language Specifics
  
  *** 1-3 links providing links to learn the language or documentation
  [Learning Python](https://realpython.com/)
  [Flask Quickstart Docs](https://flask.palletsprojects.com/en/3.0.x/quickstart/)
  [Check out Django As Well](https://www.djangoproject.com/)

#### Summary

  Python is well known for how productive it makes even novice programmers. There is a package, class, or method/helper function for pretty much anything you can imagine. The community is strong, and even after years of programming in Python, one can discover features of the language that they never knew about previously. It is a language with an infinite amount of use cases, and can excel at all of them. 
  
  Python's greatest strength is also its biggest potential drawback, especially for younger/less experienced developers. It is easy to make things "work", but much harder to know if your code is optimized or type safe due to the ease with which you can develop in the language. Libraries like [mypy](https://mypy.readthedocs.io/en/stable/cheat_sheet_py3.html) help to mitigate these drawbacks, but if your top priority is bulletproof, laser-fast code, there might be better options.
