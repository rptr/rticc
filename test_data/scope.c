int main()
{
    int x = 123;

    {
        int x = 55;
        return x;
    }

    return 0;
}