int main()
{
    int x = 9;
    {
        int x = 100;
    }
    return x;
}