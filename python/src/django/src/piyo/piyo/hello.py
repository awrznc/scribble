from django.shortcuts import render

def print(request):
    return render(request, 'index.html')
