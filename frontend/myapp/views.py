from django.shortcuts import render
from .forms import LoginForm
from django.http import HttpResponse

# Home page view
def home(request):
    if request.method == 'POST':
        form = LoginForm(request.POST)
        if form.is_valid():
            # Process the data in form.cleaned_data as required
            # Redirect to a new URL or indicate login was successful
            return HttpResponse('Login successful!')
    else:
        form = LoginForm()

    return render(request, 'home.html', {'form': form})
