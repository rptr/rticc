int main()
{
    int x = 0;

    for (x = 0; x < 7; x = x + 1)
    {
        if (x == 0)
        {
            x = 100;
            continue;
        }
    }

    return x;
}