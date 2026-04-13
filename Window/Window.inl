template<typename render>
void Window::MessageLoopRun(render renderfunc)
{
    MSG msg = { };

    while (msg.message != WM_QUIT)
    {
        if (PeekMessage(&msg, 0, 0, 0, PM_REMOVE))
        {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        else
        {
            renderfunc();
            Sleep(10); // for safety
        }

    }
}