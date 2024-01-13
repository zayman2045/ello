from django.urls import path
from . import views

urlpatterns = [
    path('', views.home, name='home'),
    path('friends', views.dashboard, name='dashboard'),
    path('friends/create', views.assistant_create, name='assistant_create'),
    path('friends/<str:assistant_id>', views.assistant_dashboard, name='assistant_dashboard'),
    path('friends/<str:assistant_id>/delete', views.assistant_delete, name='assistant_delete'),
    path('friends/<str:assistant_id>/edit', views.assistant_edit, name='assistant_edit'),
]