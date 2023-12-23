from django.shortcuts import render

# Home page view
def home(request):
    return render(request, 'home.html')

def dashboard(request):
    return render(request, 'dashboard.html')

def build(request):
    return render(request, 'build.html')