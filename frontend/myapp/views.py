import requests
from django.shortcuts import render

def home(request):
    return render(request, 'home.html')

def dashboard(request):
    # Make a GET request to the specified URL
    response = requests.get('http://localhost:8080/assistants')

    # Parse the response as JSON
    data = response.json()

    # Pass the data to the template
    return render(request, 'dashboard.html', {'data': data})

def build(request):
    return render(request, 'build.html')